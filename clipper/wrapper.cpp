#include "wrapper.h"
#include "clipper.hpp"
#include <iostream>
#include <queue>

using ClipperLib::IntPoint;
using ClipperLib::Paths;
using ClipperLib::PolyNode;
using ClipperLib::PolyTree;

ClipperLib::Path get_path(const Path& path)
{
    ClipperLib::Path clipper_path;
    for (size_t i = 0; i < path.vertices_count; ++i)
    {
        clipper_path << IntPoint(path.vertices[i][0], path.vertices[i][1]);
    }
    return clipper_path;
}

std::pair<Paths, std::vector<bool>> get_polygon_paths(const Polygon& polygon)
{
    Paths paths(polygon.paths_count);
    std::vector<bool> closed(polygon.paths_count);
    for (size_t i = 0; i < polygon.paths_count; ++i)
    {
        paths[i] = get_path(polygon.paths[i]);
        closed[i] = polygon.paths[i].closed;
    }
    return std::make_pair(paths, closed);
}

void add_paths(ClipperLib::Clipper& c, const Polygons& polygons)
{
    for (size_t i = 0; i < polygons.polygons_count; ++i)
    {
        auto& polygon = polygons.polygons[i];
        auto paths_closed = get_polygon_paths(polygon);
        Paths &paths = paths_closed.first;
        std::vector<bool> &closed = paths_closed.second;
        for (size_t i = 0; i < paths.size(); ++i) {
          c.AddPath(paths[i], ClipperLib::PolyType(polygon.type), closed[i]);
        }
    }
}

void add_paths(ClipperLib::ClipperOffset& c, JoinType join_type, EndType end_type, const Polygons& polygons)
{
    for (size_t i = 0; i < polygons.polygons_count; ++i)
    {
        auto& polygon = polygons.polygons[i];
        auto paths_closed = get_polygon_paths(polygon);
        Paths &paths = paths_closed.first;
        c.AddPaths(paths, ClipperLib::JoinType(join_type), ClipperLib::EndType(end_type));
    }
}

Path get_path_from_node(const PolyNode& node)
{
    Path path;
    path.vertices_count = node.Contour.size();
    path.vertices = new Vertice[path.vertices_count];
    path.closed = !node.IsOpen();
    for (size_t i = 0; i < path.vertices_count; ++i)
    {
        path.vertices[i][0] = node.Contour[i].X;
        path.vertices[i][1] = node.Contour[i].Y;
    }
    return path;
}

Path get_path_from_closed_clipperlib_path(ClipperLib::Path &clipper_path)
{
    Path path;
    path.vertices_count = clipper_path.size();
    path.vertices = new Vertice[path.vertices_count];
    path.closed = true;
    for (size_t i = 0; i < path.vertices_count; ++i)
    {
        path.vertices[i][0] = clipper_path[i].X;
        path.vertices[i][1] = clipper_path[i].Y;
    }
    return path;
}

Polygon get_polygon_from_closed_clipperlib_paths(ClipperLib::Paths &clipper_paths)
{
    Polygon polygon;
    polygon.type = ptSubject;
    polygon.paths_count = clipper_paths.size();
    polygon.paths = new Path[polygon.paths_count];
    for (int i = 0; i < polygon.paths_count; ++i)
    {
        polygon.paths[i] = get_path_from_closed_clipperlib_path(clipper_paths[i]);
    }

    return polygon;
}

Polygon get_polygon_from_node(
    const PolyNode* node, std::queue<const PolyNode*>& node_queue)
{
    Polygon polygon;
    polygon.type = ptSubject;
    polygon.paths_count = node->ChildCount() + 1;
    polygon.paths = new Path[polygon.paths_count];
    polygon.paths[0] = get_path_from_node(*node);
    for (int i = 0; i < node->ChildCount(); ++i)
    {
        auto child = node->Childs[i];
        polygon.paths[i + 1] = get_path_from_node(*child);
        for (const auto grand_child : child->Childs)
        {
            node_queue.push(grand_child);
        }
    }
    return polygon;
}

Polygons get_polygons_from_tree(const PolyTree& tree)
{
    std::queue<const PolyNode*> node_queue;
    std::vector<Polygon> polygon_vector;

    for (const auto node : tree.Childs)
    {
        node_queue.push(node);
    }

    while (!node_queue.empty())
    {
        const auto node = node_queue.front();
        node_queue.pop();
        polygon_vector.push_back(get_polygon_from_node(node, node_queue));
    }

    Polygons polygons;
    polygons.polygons_count = polygon_vector.size();
    polygons.polygons = new Polygon[polygons.polygons_count];
    std::copy(polygon_vector.begin(), polygon_vector.end(), polygons.polygons);

    return polygons;
}

Polygons execute(
    ClipType clip_type,
    Polygons polygons,
    PolyFillType subject_fill_type,
    PolyFillType clip_fill_type)
{
    ClipperLib::Clipper c;
    add_paths(c, polygons);
    PolyTree solution;
    c.Execute(
        ClipperLib::ClipType(clip_type), solution,
        ClipperLib::PolyFillType(subject_fill_type),
        ClipperLib::PolyFillType(clip_fill_type));
    return get_polygons_from_tree(solution);
}

Polygons offset(
    double miter_limit,
    double round_precision,
    JoinType join_type,
    EndType end_type,
    Polygons polygons,
    double delta)
{
  ClipperLib::ClipperOffset c(miter_limit, round_precision);
  add_paths(c, join_type, end_type, polygons);
  PolyTree solution;
  c.Execute(solution, delta);
  return get_polygons_from_tree(solution);
}

void free_path(Path path)
{
    delete[] path.vertices;
}

void free_polygon(Polygon polygon)
{
    for (size_t i = 0; i < polygon.paths_count; ++i)
    {
        free_path(polygon.paths[i]);
    }
    delete[] polygon.paths;
}

void free_polygons(Polygons polygons)
{
    for (size_t i = 0; i < polygons.polygons_count; ++i)
    {
        free_polygon(polygons.polygons[i]);
    }
    delete[] polygons.polygons;
}
