
void __fastcall FUN_004361d0(int *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar2;
  
  if (*param_1 == 0) {
    iVar2 = FUN_0041d6d0();
    *param_1 = iVar2;
  }
  else {
    (**(code **)(*(int *)param_1[4] + 0x28))();
    (**(code **)(*(int *)param_1[5] + 0x28))();
    bVar1 = FUN_0041dc10();
    if (CONCAT31(extraout_var,bVar1) != 0) {
      bVar1 = FUN_004888c0(param_1[4]);
      if (CONCAT31(extraout_var_00,bVar1) != 0) {
        bVar1 = FUN_004888c0(param_1[5]);
        if (CONCAT31(extraout_var_01,bVar1) != 0) {
          FUN_0041dbf0();
          return;
        }
      }
    }
  }
  return;
}

