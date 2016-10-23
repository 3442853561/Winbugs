//winbugs.c
#include <stdlib.h>
#include <windows.h>

void wbMsgbox(char* text,char* title)
{
	MessageBox (NULL,text,title,0);
}

int screenWidth()
{
	return GetSystemMetrics(SM_CXSCREEN);
}

int screenHeight()
{
	return GetSystemMetrics(SM_CYSCREEN);
}