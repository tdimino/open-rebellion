
void __fastcall FUN_005dab90(int *param_1)

{
  undefined4 uVar1;
  uint *puVar2;
  void *pvVar3;
  int iVar4;
  int *piVar5;
  undefined *puVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654bd7;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(6);
  puVar2 = (uint *)FUN_005ff440((int)param_1);
  pvVar3 = (void *)FUN_00618b70(0x114);
  piVar5 = (int *)0x0;
  local_4 = 0;
  if (pvVar3 != (void *)0x0) {
    piVar5 = FUN_00602150(pvVar3,uVar1,0x12,0x13,0x30,0x22,param_1,0xdd01,0x47d,0x47e,puVar2,0,1);
  }
  local_4 = 0xffffffff;
  (**(code **)(*param_1 + 0x4c))(piVar5,0,&DAT_006bcd2c);
  iVar4 = FUN_00596a90();
  if (iVar4 == 0) {
    (**(code **)(*piVar5 + 0x10))();
    (**(code **)(*piVar5 + 8))();
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    piVar5 = (int *)0x0;
  }
  else {
    piVar5 = FUN_00602150(pvVar3,uVar1,0xe,0x5d,0x2c,0x19,param_1,0xdd02,0x47f,0x480,puVar2,0,1);
  }
  (**(code **)(*param_1 + 0x4c))(piVar5,0,&DAT_006bcd4c);
  iVar4 = FUN_00596bc0();
  if (iVar4 != 0) {
    iVar4 = FUN_00596ab0();
    if (iVar4 != 1) goto LAB_005dacb0;
  }
  (**(code **)(*piVar5 + 0x10))();
  (**(code **)(*piVar5 + 8))();
LAB_005dacb0:
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    piVar5 = (int *)0x0;
  }
  else {
    piVar5 = FUN_00602150(pvVar3,uVar1,0x57,0x5d,0x2d,0x19,param_1,0xdd03,0x481,0x482,puVar2,0,1);
  }
  FUN_00603150(piVar5,4,0x482);
  FUN_006030c0(piVar5,8);
  (**(code **)(*param_1 + 0x4c))(piVar5,0,&DAT_006bcd24);
  iVar4 = FUN_00596ab0();
  if (iVar4 == 1) {
    (**(code **)(*piVar5 + 0x10))();
    (**(code **)(*piVar5 + 8))();
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x17,0xb8,0x34,0x21,param_1,0xdd04,0x483,0x484,puVar2,0,1);
  }
  puVar6 = &DAT_006bccb8;
  (**(code **)(*param_1 + 0x4c))(pvVar3,0);
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x61,0xb8,0x1d,0x21,param_1,0xdd05,0x485,0x486,puVar2,0,1);
  }
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bcc88);
  ExceptionList = puVar6;
  return;
}

