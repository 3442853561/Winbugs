//winbugs.c
#include <stdlib.h>
#include <string.h>
#include <windows.h>
typedef struct cWindow
{
	char *caption;
	HWND hwnd;
	WNDCLASSEX wndclass;
	int showstyle;
}cWindow;
LRESULT CALLBACK WndProc(HWND hwnd, UINT Message, WPARAM wParam, LPARAM lParam);
int wbMsgbox(char* text,char* title,int style)
{
	return MessageBox (NULL,text,title,style);
}

int screenWidth()
{
	return GetSystemMetrics(SM_CXSCREEN);
}

int screenHeight()
{
	return GetSystemMetrics(SM_CYSCREEN);
}

void wbAbout(char* appName,char* appString)
{
	ShellAboutA(NULL,appName,appString,0);
}

cWindow cWindow_new(char name[128])
{
	int flag = 0;
	cWindow foo;
	HINSTANCE hInstance = (HINSTANCE)GetModuleHandle(NULL);
	foo.wndclass.cbSize        = sizeof(WNDCLASSEX);
	foo.wndclass.lpfnWndProc   = WndProc;
	foo.wndclass.hInstance     = hInstance;
	foo.wndclass.hCursor       = LoadCursor(NULL, IDC_ARROW);
	foo.wndclass.hbrBackground = (HBRUSH)(COLOR_WINDOW+1);
	foo.wndclass.lpszClassName = "Windofoo.wndclasslass";
	foo.wndclass.hIcon		   = LoadIcon(NULL, IDI_APPLICATION); 
	foo.wndclass.hIconSm       = LoadIcon(NULL, IDI_APPLICATION); 
	foo.caption                = NULL;
	if(!RegisterClassEx(&foo.wndclass))
	{
		flag = wbMsgbox("Window Registration Failed.","Error",18);
		while(flag == 4)
			if(!RegisterClassEx(&foo.wndclass))
				flag = wbMsgbox("Window Registration Failed.","Error",18);
			else
				flag = 1;
		if(flag != 1)
		{
			foo.hwnd = NULL;
			return foo;
		} 
	}
	foo.hwnd = CreateWindowEx(WS_EX_CLIENTEDGE,TEXT(name),TEXT("Unnamed Form"),WS_VISIBLE|WS_OVERLAPPEDWINDOW,
		CW_USEDEFAULT, /* x */
		CW_USEDEFAULT, /* y */
		640, /* width */
		480, /* height */
		NULL,NULL,hInstance,NULL);
	if(foo.hwnd == NULL) {
		wbMsgbox("Window Creation Failed.","Error",18);
	}
	return foo;
}

void cWindow_show(cWindow foo)
{
	ShowWindow(foo.hwnd, foo.showstyle);
	UpdateWindow(foo.hwnd);
}

LRESULT CALLBACK WndProc(HWND hwnd, UINT Message, WPARAM wParam, LPARAM lParam){return 0;}