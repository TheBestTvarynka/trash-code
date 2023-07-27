#include <iostream>
#include <Windows.h>
#include "imgur_api.h"

using std::cout;
using std::endl;

int main()
{
    cout << sizeof(bool) << endl;
    HMODULE imgur = LoadLibraryA("imgur_api.dll");
    if (!imgur)
    {
        cout << "Can not find imgur_api.dll" << endl;
        return -1;
    }

    FARPROC ImgurInitClientAddress = GetProcAddress(imgur, "ImgurInitClient");
    if (!ImgurInitClientAddress)
    {
        cout << "Can not find ImgurInitClient in DLL handle" << endl;
        return -1;
    }
    FARPROC ImgurGetCommentAddress = GetProcAddress(imgur, "ImgurGetComment");
    if (!ImgurGetCommentAddress)
    {
        cout << "Can not find ImgurGetComment in DLL handle" << endl;
        return -1;
    }

    const char* client_id = "3d8012c2f66acfb";
    const char* client_secret = "708d779959d043dd6da2d158abaa022931f708a8";

    void* context = ((ImgurInitClientFn*)ImgurInitClientAddress)(client_id, client_secret);

    unsigned long long comment_id = 1911999579;
    FiiComment* comment = nullptr;
    uint32_t status = ((ImgurGetCommentFn*)ImgurGetCommentAddress)(context, comment_id, &comment);

    if (!status)
    {
        cout << "Success! Comment data:\n";
        cout << "id: " << comment->data.id << endl;
        cout << "image_id: " << comment->data.image_id << endl;
        cout << "comment: " << comment->data.comment << endl;
        cout << "author: " << comment->data.author << endl;
        cout << "author_id: " << comment->data.author_id << endl;
        cout << "on_album: " << comment->data.on_album << endl;
        cout << "album_cover: " << comment->data.album_cover << endl;
        cout << "ups: " << comment->data.ups << endl;
        cout << "downs: " << comment->data.downs << endl;
        cout << "points: " << comment->data.points << endl;
        cout << "datetime: " << comment->data.datetime << endl;
        cout << "parent_id: " << comment->data.parent_id << endl;
        cout << "deleted: " << comment->data.deleted << endl;
        cout << "is_voted: " << comment->data.is_voted << endl;
        cout << "vote: " << comment->data.vote << endl;
        cout << "platform: " << comment->data.platform << endl;
        cout << "has_admin_badge: " << comment->data.has_admin_badge << endl;
        cout << "children_len: " << comment->data.children_len << endl;

        cout << "status: " << comment->status << endl;
        cout << "success: " << comment->success << endl;
    }
    else
    {
        cout << "Error: " << status << endl;
    }

}
