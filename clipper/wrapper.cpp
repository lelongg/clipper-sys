#include "wrapper.h"
#include "clipper.hpp"
#include <queue>

using ClipperLib::IntPoint;
using ClipperLib::Path;
using ClipperLib::Paths;
using ClipperLib::PolyNode;
using ClipperLib::PolyTree;

int get_polygon_paths(
    const cInt* paths_vertices,
    int paths_count,
    const int* paths_sizes,
    Paths& paths)
{
    paths = Paths(paths_count);
    int vertice_index = 0;
    for (int i = 0; i < paths_count; ++i)
    {
        auto paths_size = paths_sizes[i];
        for (int j = 0; j < paths_size; ++j)
        {
            paths[i] << IntPoint(
                paths_vertices[vertice_index],
                paths_vertices[vertice_index + 1]);
            vertice_index += 2;
        }
    }
    return vertice_index;
}

void add_paths(ClipperLib::Clipper& c, const Polygons& polygons)
{
    int vertice_index = 0;
    int path_index = 0;
    for (int i = 0; i < polygons.count; ++i)
    {
        Paths paths;
        vertice_index += get_polygon_paths(
            polygons.paths_vertices + vertice_index, polygons.paths_count[i],
            polygons.paths_sizes + path_index, paths);
        c.AddPaths(
            paths, ClipperLib::PolyType(polygons.paths_types[i]),
            polygons.paths_closed[i]);
        path_index += polygons.paths_count[i];
    }
}

void path_and_vertice_count(
    const PolyTree& tree,
    int& polygon_count,
    int& path_count,
    int& vertice_count)
{
    std::queue<const PolyNode*> node_queue;

    for (const auto node : tree.Childs)
    {
        node_queue.push(node);
    }

    while (!node_queue.empty())
    {
        const auto node = node_queue.front();

        for (const auto child : node->Childs)
        {
            for (const auto grand_child : child->Childs)
            {
                node_queue.push(grand_child);
            }

            vertice_count += child->Contour.size();
            ++path_count;
        }

        vertice_count += node->Contour.size();
        ++path_count;
        ++polygon_count;
    }
}

void add_path_to_polygons(Polygons& polygons, const Path& path)
{
    auto& vertice_index = polygons.vertices_count;
    for (int i = 0; i < path.size(); ++i)
    {
        polygons.paths_vertices[vertice_index++] = path[i].X;
        polygons.paths_vertices[vertice_index++] = path[i].Y;
    }
    polygons.paths_sizes[polygons.paths_count[polygons.count]++] = path.size();
}

void add_node_to_polygons(
    Polygons& polygons,
    const PolyNode* node,
    std::queue<const PolyNode*>& node_queue)
{
    int path_index = polygons.paths_count[polygons.count];
    add_path_to_polygons(polygons, node->Contour);
    polygons.paths_types[path_index] = ptSubject;
    polygons.paths_closed[path_index] = !node->IsOpen();

    for (const auto child : node->Childs)
    {
        int path_index = polygons.paths_count[polygons.count];
        add_path_to_polygons(polygons, child->Contour);
        polygons.paths_types[path_index] = ptSubject;
        polygons.paths_closed[path_index] = !child->IsOpen();

        for (const auto grand_child : child->Childs)
        {
            node_queue.push(grand_child);
        }
    }

    ++polygons.count;
}

Polygons add_tree_to_polygons(Polygons& polygons, const PolyTree& tree)
{
    std::queue<const PolyNode*> node_queue;

    for (const auto node : tree.Childs)
    {
        node_queue.push(node);
    }

    while (!node_queue.empty())
    {
        const auto node = node_queue.front();
        node_queue.pop();
        add_node_to_polygons(polygons, node, node_queue);
    }
}

Polygons tree_to_polygons(const PolyTree& tree)
{
    int polygon_count, path_count, vertice_count;
    path_and_vertice_count(tree, polygon_count, path_count, vertice_count);

    Polygons polygons;
    polygons.paths_vertices = new cInt[vertice_count];
    polygons.paths_count = new int[polygon_count];
    polygons.paths_sizes = new int[path_count];
    polygons.paths_types = new PolyType[path_count];
    polygons.paths_closed = new int[path_count];

    add_tree_to_polygons(polygons, tree);

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
        ClipperLib::ClipType(ctIntersection), solution,
        ClipperLib::PolyFillType(subject_fill_type),
        ClipperLib::PolyFillType(clip_fill_type));
    return tree_to_polygons(solution);
}