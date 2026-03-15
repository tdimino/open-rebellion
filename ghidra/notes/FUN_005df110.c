
void __fastcall FUN_005df110(int *param_1)

{
  bool bVar1;
  undefined4 uVar2;
  int iVar3;
  uint *puVar4;
  int iVar5;
  void *pvVar6;
  int *piVar7;
  void *unaff_EBX;
  uint uVar8;
  uint uStack_20;
  uint uStack_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00654d22;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar2 = FUN_006037f0(6);
  iVar3 = FUN_00401060();
  bVar1 = true;
  puVar4 = (uint *)FUN_005ff440((int)param_1);
  if ((int *)param_1[0x30] != (int *)0x0) {
    iVar5 = (**(code **)(*(int *)param_1[0x30] + 4))();
    if (iVar5 == 4) {
      bVar1 = false;
    }
  }
  if (iVar3 == 1) {
    uVar8 = 0x49b;
    uStack_20 = 0x49c;
  }
  else {
    uVar8 = 0x49e;
    uStack_20 = 0x49f;
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  uStack_4 = 0;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_00602150(pvVar6,uVar2,0x12,0x81,0x2e,0x1a,param_1,0xdd03,uVar8,uStack_20,puVar4,0,1
                         );
  }
  uStack_4 = 0xffffffff;
  if (pvVar6 != (void *)0x0) {
    FUN_006030c0(pvVar6,8);
    FUN_00603150(pvVar6,4,uStack_20);
    (**(code **)(*param_1 + 0x4c))(pvVar6,0,&DAT_006bd214);
    param_1[0x27] = (int)pvVar6;
  }
  if (iVar3 == 1) {
    uVar8 = 0x492;
    uStack_20 = 0x493;
    uStack_1c = 0x494;
  }
  else {
    uVar8 = 0x495;
    uStack_20 = 0x496;
    uStack_1c = 0x497;
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  uStack_4 = 1;
  if (pvVar6 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_00602150(pvVar6,uVar2,0x56,0x81,0x2e,0x1a,param_1,0xdd05,uVar8,uStack_20,
                                 puVar4,0,1);
  }
  uStack_4 = 0xffffffff;
  if (piVar7 != (int *)0x0) {
    FUN_006030c0(piVar7,8);
    FUN_00603150(piVar7,4,uStack_20);
    FUN_00603150(piVar7,2,uStack_1c);
    (**(code **)(*param_1 + 0x4c))(piVar7,0,&DAT_006bd098);
    param_1[0x29] = (int)piVar7;
    if (bVar1) {
      (**(code **)(*piVar7 + 0x10))();
    }
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  uStack_4 = 2;
  if (pvVar6 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_00602150(pvVar6,uVar2,0x56,0x9f,0x2e,0x1a,param_1,0xdd06,0x498,0x499,puVar4,
                                 0,1);
  }
  uStack_4 = 0xffffffff;
  if (piVar7 != (int *)0x0) {
    FUN_006030c0(piVar7,8);
    FUN_00603150(piVar7,4,0x499);
    FUN_00603150(piVar7,2,0x49a);
    (**(code **)(*param_1 + 0x4c))(piVar7,0,&DAT_006bd0c4);
    param_1[0x2a] = (int)piVar7;
    (**(code **)(*piVar7 + 0x10))();
  }
  if (iVar3 == 1) {
    uVar8 = 0x4a7;
    uStack_20 = 0x4a8;
  }
  else {
    uVar8 = 0x4aa;
    uStack_20 = 0x4ab;
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  uStack_4 = 3;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_00602150(pvVar6,uVar2,0x12,0x9f,0x2e,0x1a,param_1,0xdd04,uVar8,uStack_20,puVar4,0,1
                         );
  }
  uStack_4 = 0xffffffff;
  if (pvVar6 != (void *)0x0) {
    FUN_006030c0(pvVar6,8);
    FUN_00603150(pvVar6,4,uStack_20);
    (**(code **)(*param_1 + 0x4c))(pvVar6,0,&DAT_006bd0e0);
    param_1[0x28] = (int)pvVar6;
  }
  pvVar6 = (void *)FUN_00618b70(0x114);
  uStack_4 = 4;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_00602150(pvVar6,uVar2,0x42,0xca,0x1b,0x19,param_1,0xdd01,0x459,0x45a,puVar4,0,1);
  }
  uStack_4 = 0xffffffff;
  (**(code **)(*param_1 + 0x4c))(pvVar6,0,&DAT_006bd094);
  pvVar6 = (void *)FUN_00618b70(0x114);
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_00602150(pvVar6,uVar2,0x6a,0xca,0x1b,0x19,param_1,0xdd02,0x45b,0x45c,puVar4,0,1);
  }
  (**(code **)(*param_1 + 0x4c))(pvVar6,0,&DAT_006bd054);
  ExceptionList = unaff_EBX;
  return;
}

