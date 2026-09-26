
LRESULT FUN_006007b0(int *param_1,UINT param_2,uint param_3,LPARAM param_4)

{
  LRESULT LVar1;
  int iVar2;
  
  if (param_2 == 0x18) {
    if (param_3 != 0) {
      for (iVar2 = FUN_005f5060((int)(param_1 + 0x1d)); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10))
      {
        (**(code **)(**(int **)(iVar2 + 0x1c) + 4))(5);
      }
    }
    return 0;
  }
  if (param_2 == 0x205) {
    LVar1 = (**(code **)(*param_1 + 0x1c))((int)(short)param_4,(int)param_4._2_2_,param_1);
    return LVar1;
  }
  if (param_2 == 0x405) {
    FUN_00600f90(param_1,param_3);
    return 0;
  }
  LVar1 = DefWindowProcA((HWND)param_1[6],param_2,param_3,param_4);
  return LVar1;
}

