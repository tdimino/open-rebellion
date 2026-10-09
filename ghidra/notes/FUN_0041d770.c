
void __cdecl FUN_0041d770(uint param_1,int param_2)

{
  int iVar1;
  
  iVar1 = FUN_00422ca0();
  if (iVar1 != 0) {
    PostMessageA(*(HWND *)(iVar1 + 0x18),0x468,0xd,param_2 << 0x10 | param_1 & 0xffff);
  }
  return;
}

