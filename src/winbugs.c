//winbugs.c
#include <stdlib.h>
#include <windows.h>

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