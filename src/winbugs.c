//winbugs.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>

typedef struct cWindow
{
	char *caption;
	HWND hwnd;
	WNDCLASSEX wndclass;
	MSG msg;
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

cWindow cWindow_new(char *name)
{
	int flag = 0;
	HINSTANCE hInstance=GetModuleHandle(NULL);
	cWindow foo;
	memset(&foo.wndclass,0,sizeof(foo.wndclass));
	foo.wndclass.cbSize        = sizeof(WNDCLASSEX);
	foo.wndclass.lpfnWndProc   = WndProc;
	foo.wndclass.hInstance     = hInstance;
	foo.wndclass.hCursor       = LoadCursor(NULL, IDC_ARROW);
	foo.wndclass.hbrBackground = (HBRUSH)(COLOR_WINDOW+1);
	foo.wndclass.lpszClassName = name;
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
	foo.hwnd = CreateWindowEx(WS_EX_CLIENTEDGE,name,"Unnamed Form",WS_VISIBLE|WS_OVERLAPPEDWINDOW,
		CW_USEDEFAULT, 
		CW_USEDEFAULT, 
		550, 
		400, 
		NULL,NULL,hInstance,NULL);
	if(foo.hwnd == NULL) {
		flag = wbMsgbox("Window Creation Failed.","Error",18);
		while(flag == 4)
		{
			foo.hwnd = CreateWindowEx(WS_EX_CLIENTEDGE,name,"Unnamed Form",WS_VISIBLE|WS_OVERLAPPEDWINDOW,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			550,
			400,
			NULL,NULL,hInstance,NULL);
			if(foo.hwnd == NULL)
				flag = wbMsgbox("Window Creation Failed.","Error",18);
			else
				flag = 1;
		}
	}
	while(GetMessage(&foo.msg, NULL, 0, 0) > 0) {
		TranslateMessage(&foo.msg);
		DispatchMessage(&foo.msg);
	}
	return foo;
}

LRESULT CALLBACK WndProc(HWND hwnd, UINT Message, WPARAM wParam, LPARAM lParam) {
	switch(Message) {
		case WM_GETMINMAXINFO: {
			break;
		}
		case WM_HELP: {
			break;
		}
		case WM_DESTROY: {
			exit(0);
			break;
		}
		default:
			return DefWindowProc(hwnd, Message, wParam, lParam);
	}
	return 0;
}
