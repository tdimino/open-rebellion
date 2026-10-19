
undefined4 __fastcall FUN_0042edc0(void *param_1)

{
  undefined1 uVar1;
  void *pvVar2;
  undefined3 extraout_var;
  undefined4 uVar3;
  
  uVar3 = 0;
  pvVar2 = (void *)thunk_FUN_005f5060((int)param_1);
  if (pvVar2 != (void *)0x0) {
    uVar1 = FUN_00474260((int)pvVar2);
    if (CONCAT31(extraout_var,uVar1) == 0) {
      uVar3 = *(undefined4 *)((int)pvVar2 + 0x18);
      FUN_004742b0(pvVar2);
      FUN_005f54a0(param_1,*(uint *)((int)pvVar2 + 0x18));
    }
  }
  return uVar3;
}

