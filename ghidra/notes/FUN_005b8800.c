
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005b8800(int param_1)

{
  uint uVar1;
  undefined4 uVar2;
  int iVar3;
  uint uVar4;
  void *pvVar5;
  int *piVar6;
  float10 fVar7;
  int iVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006534cb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar3 = (**(code **)(**(int **)(param_1 + 0x1c) + 0x10))();
  if (iVar3 == 0) {
    ExceptionList = pvStack_c;
    return;
  }
  if (*(float *)(param_1 + 0x14) == _DAT_0066c690) {
    iVar3 = FUN_005aaf90();
    uVar1 = *(uint *)(iVar3 + 0x87c);
    iVar8 = 8;
    iVar3 = 0;
    FUN_005aaf90();
    uVar4 = FUN_005a8a70(iVar3,iVar8);
    fVar7 = FUN_005b8b00(param_1);
    fVar7 = fVar7 + (float10)uVar4 + (float10)uVar1;
  }
  else {
    iVar3 = FUN_005aaf90();
    if ((float)*(uint *)(iVar3 + 0x87c) < *(float *)(param_1 + 0x14)) {
      ExceptionList = pvStack_c;
      return;
    }
    iVar3 = FUN_005c6070(*(int **)(param_1 + 0x1c));
    pvVar5 = (void *)FUN_00618b70(0x220);
    uStack_4 = 0;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      uVar2 = *(undefined4 *)(iVar3 + 0xec);
      iVar3 = FUN_005aaf90();
      pvVar5 = FUN_005ce500(pvVar5,(float)*(uint *)(iVar3 + 0x87c),0x19,*(undefined4 *)(param_1 + 4)
                            ,uVar2,(uint *)0x0);
    }
    uStack_4 = 0xffffffff;
    piVar6 = (int *)FUN_005aaf90();
    (**(code **)(*piVar6 + 0x2c))(pvVar5,1);
    if (*(int *)(param_1 + 8) == 0) {
      ExceptionList = pvStack_c;
      return;
    }
    iVar3 = FUN_005aaf90();
    uVar1 = *(uint *)(iVar3 + 0x87c);
    iVar8 = 8;
    iVar3 = 0;
    FUN_005aaf90();
    uVar4 = FUN_005a8a70(iVar3,iVar8);
    fVar7 = FUN_005b8b00(param_1);
    fVar7 = fVar7 + (float10)uVar4 + (float10)uVar1;
  }
  *(float *)(param_1 + 0x14) = (float)fVar7;
  ExceptionList = pvStack_c;
  return;
}

