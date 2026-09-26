
void __thiscall FUN_005b5ad0(void *this,undefined4 *param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int *piVar6;
  int iVar7;
  uint uVar8;
  undefined4 local_10 [4];
  
  uVar8 = *(uint *)((int)this + 0x174);
  if (uVar8 == 0) {
    iVar7 = *(int *)((int)this + 0x168);
    puVar5 = local_10;
    pvVar4 = (void *)FUN_005aaf90();
    puVar5 = (undefined4 *)FUN_005a7490(pvVar4,puVar5,iVar7);
  }
  else {
    if (*(int *)((int)this + 0x140) == 1) {
      pvVar4 = (void *)FUN_00596bd0();
      piVar6 = (int *)FUN_005a6470(pvVar4,uVar8);
      iVar7 = *piVar6;
    }
    else {
      pvVar4 = (void *)FUN_00596bd0();
      piVar6 = (int *)FUN_005a6460(pvVar4,uVar8);
      iVar7 = *piVar6;
    }
    puVar5 = (undefined4 *)(**(code **)(iVar7 + 0x40))(local_10);
  }
  uVar1 = puVar5[1];
  uVar2 = puVar5[2];
  uVar3 = puVar5[3];
  *param_1 = *puVar5;
  param_1[2] = uVar2;
  param_1[1] = uVar1;
  param_1[3] = uVar3;
  return;
}

