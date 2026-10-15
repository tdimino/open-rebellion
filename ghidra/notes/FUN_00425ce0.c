
void __thiscall FUN_00425ce0(int param_1,LPARAM param_2)

{
  *(LPARAM *)(param_1 + 0x498) = param_2;
  PostMessageA(*(HWND *)(param_1 + 0x18),0x482,1,param_2);
  return;
}

