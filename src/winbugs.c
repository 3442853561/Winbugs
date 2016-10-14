//winbugs.c
#include <windows.h>
void wbMsgbox(char* text,char* title)
{
	MessageBox (NULL,TEXT(text),TEXT(title),0);
}