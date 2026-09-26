
int __fastcall FUN_0040f450(int *param_1)

{
  bool bVar1;
  undefined *puVar2;
  undefined3 extraout_var;
  int *piVar3;
  int iVar4;
  void *pvVar5;
  char *pcVar6;
  int iVar7;
  int local_14;
  void *pvStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a3eb;
  local_c = ExceptionList;
  local_14 = 0x11;
  if ((param_1[0x62] != 0) && (param_1[100] == 0)) {
    ExceptionList = &local_c;
    puVar2 = FUN_00597210(0,&local_14);
    if (local_14 == 0) {
      bVar1 = FUN_0040f5d0(param_1,(int)puVar2);
      if (CONCAT31(extraout_var,bVar1) != 0) {
        for (piVar3 = (int *)(**(code **)(**(int **)(param_1[0x62] + 0xa0) + 8))();
            piVar3 != (int *)0x0; piVar3 = (int *)(**(code **)(*piVar3 + 0xc))()) {
          if ((undefined4 *)piVar3[0x15] != (undefined4 *)0x0) {
            (*(code *)**(undefined4 **)piVar3[0x15])(1);
          }
        }
        FUN_00609920((void *)param_1[0x62]);
        for (iVar4 = thunk_FUN_005f5060((int)puVar2); iVar4 != 0; iVar4 = *(int *)(iVar4 + 0x10)) {
          pvStack_10 = (void *)FUN_00618b70(0x38);
          uStack_4 = 0;
          if (pvStack_10 == (void *)0x0) {
            pvVar5 = (void *)0x0;
          }
          else {
            pvVar5 = FUN_005f7be0(pvStack_10,iVar4);
          }
          iVar7 = 1;
          uStack_4 = 0xffffffff;
          pcVar6 = (char *)FUN_00583c40(iVar4 + 0x2c);
          FUN_00609790((void *)param_1[0x62],0,0,0,pcVar6,iVar7,pvVar5);
        }
        iVar4 = FUN_00609eb0(param_1[0x62]);
        if (iVar4 != 0) {
          FUN_00609500((void *)param_1[0x62],*(int *)(iVar4 + 0xc),1);
        }
        if (param_1[0x50] != 0) {
          iVar4 = FUN_00609eb0(param_1[0x62]);
          if (iVar4 == 0) {
            (**(code **)(*(int *)param_1[0x50] + 0x10))();
          }
          else {
            (**(code **)(*(int *)param_1[0x50] + 0xc))();
          }
        }
      }
    }
    else {
      FUN_00410b10(param_1);
    }
  }
  ExceptionList = local_c;
  return local_14;
}

