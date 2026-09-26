
void __fastcall FUN_0040f350(int param_1)

{
  int *piVar1;
  undefined *puVar2;
  int iVar3;
  void *pvVar4;
  char *pcVar5;
  int iVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a3cb;
  local_c = ExceptionList;
  if (*(int *)(param_1 + 0x184) != 0) {
    ExceptionList = &local_c;
    for (piVar1 = (int *)(**(code **)(**(int **)(*(int *)(param_1 + 0x184) + 0xa0) + 8))();
        piVar1 != (int *)0x0; piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
      if ((undefined4 *)piVar1[0x15] != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)piVar1[0x15])(1);
      }
    }
    FUN_00609920(*(void **)(param_1 + 0x184));
    puVar2 = FUN_005971c0();
    for (iVar3 = thunk_FUN_005f5060((int)puVar2); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      pvVar4 = (void *)FUN_00618b70(0x38);
      uStack_4 = 0;
      if (pvVar4 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_005f7a50(pvVar4,iVar3);
      }
      iVar6 = 1;
      uStack_4 = 0xffffffff;
      pcVar5 = (char *)FUN_00583c40(iVar3 + 0x2c);
      FUN_00609790(*(void **)(param_1 + 0x184),0,0,0,pcVar5,iVar6,pvVar4);
    }
    iVar3 = FUN_00609eb0(*(int *)(param_1 + 0x184));
    if (iVar3 != 0) {
      FUN_00609500(*(void **)(param_1 + 0x184),*(int *)(iVar3 + 0xc),1);
    }
  }
  ExceptionList = local_c;
  return;
}

