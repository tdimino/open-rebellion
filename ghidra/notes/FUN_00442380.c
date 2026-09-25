
int * __cdecl
FUN_00442380(int param_1,undefined4 param_2,int *param_3,int param_4,undefined4 param_5)

{
  int iVar1;
  undefined4 uVar2;
  
  iVar1 = FUN_00442430();
  if (iVar1 != 0) {
    FUN_00442c90(DAT_006b28e8);
    uVar2 = FUN_005ff440(param_4);
    DAT_006b28f4 = FUN_00442860(param_1,param_2,&DAT_006b28e0,param_3,(void *)0x0,param_4,param_5,
                                uVar2);
  }
  if (DAT_006b28f4 != (int *)0x0) {
    (**(code **)(*DAT_006b28f4 + 0x2c))(5);
    SetWindowPos((HWND)DAT_006b28f4[6],(HWND)0x0,0,0,0,0,3);
    FUN_00442c20((int)DAT_006b28f4);
    DAT_006b28fc = GetFocus();
    SetFocus((HWND)DAT_006b28f4[6]);
  }
  return DAT_006b28f4;
}

