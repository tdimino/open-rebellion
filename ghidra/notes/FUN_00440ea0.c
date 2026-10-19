
void __thiscall FUN_00440ea0(void *param_1,void *param_2)

{
  void *pvVar1;
  short sVar2;
  undefined2 extraout_var;
  HWND hWnd;
  int *piVar3;
  uint uStack_4;
  
  pvVar1 = param_2;
  uStack_4 = (uint)CONCAT11(*(undefined1 *)((int)param_1 + 0xa8),*(undefined1 *)((int)param_1 + 200)
                           );
  FUN_005f4df0(param_2,&uStack_4);
  sVar2 = FUN_005f50e0((int)param_1 + 0x6c);
  param_2 = (void *)CONCAT22(extraout_var,sVar2);
  FUN_005f4df0(pvVar1,&param_2);
  for (hWnd = FUN_00440f20((int)param_1); hWnd != (HWND)0x0; hWnd = GetWindow(hWnd,3)) {
    piVar3 = (int *)FUN_00440f50(param_1,(int)hWnd);
    if (piVar3 != (int *)0x0) {
      FUN_004410a0(pvVar1,piVar3);
    }
  }
  return;
}

