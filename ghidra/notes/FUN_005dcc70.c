
void __fastcall FUN_005dcc70(int *param_1)

{
  undefined4 uVar1;
  uint *puVar2;
  void *pvVar3;
  void *this;
  undefined *puVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654c88;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(6);
  puVar2 = (uint *)FUN_005ff440((int)param_1);
  pvVar3 = (void *)FUN_00618b70(0x114);
  this = (void *)0x0;
  local_4 = 0;
  if (pvVar3 != (void *)0x0) {
    this = FUN_00602150(pvVar3,uVar1,0xd,0x9f,0x1c,0x1c,param_1,0xdd10,0x46d,0x46e,puVar2,0,1);
  }
  local_4 = 0xffffffff;
  if (this != (void *)0x0) {
    FUN_006030c0(this,8);
    FUN_00603150(this,4,0x46e);
    (**(code **)(*param_1 + 0x4c))(this,0,&DAT_006bceec);
    param_1[0x27] = (int)this;
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 1;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x2b,0x9f,0x1c,0x1c,param_1,0xdd11,0x46f,0x470,puVar2,0,1);
  }
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_006030c0(pvVar3,8);
    FUN_00603150(pvVar3,4,0x470);
    (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bd030);
    param_1[0x28] = (int)pvVar3;
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 2;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0xd,0x81,0x1c,0x1c,param_1,0xdd12,0x471,0x472,puVar2,0,1);
  }
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_006030c0(pvVar3,8);
    FUN_00603150(pvVar3,4,0x472);
    (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bcfc4);
    param_1[0x29] = (int)pvVar3;
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x2b,0x81,0x1c,0x1c,param_1,0xdd13,0x473,0x474,puVar2,0,1);
  }
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_006030c0(pvVar3,8);
    FUN_00603150(pvVar3,4,0x474);
    (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bcebc);
    param_1[0x2a] = (int)pvVar3;
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 4;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x4b,0x90,0x1c,0x1c,param_1,0xdd14,0x475,0x476,puVar2,0,1);
  }
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_006030c0(pvVar3,8);
    FUN_00603150(pvVar3,4,0x476);
    (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bcf88);
    param_1[0x2b] = (int)pvVar3;
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 5;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x6f,0x84,0x18,0x34,param_1,0xdd20,0x45d,0x45e,puVar2,0,1);
  }
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_00603150(pvVar3,4,0x45e);
    FUN_006030c0(pvVar3,8);
    (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bd01c);
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 6;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x42,0xca,0x1b,0x19,param_1,0xdd01,0x459,0x45a,puVar2,0,1);
  }
  puVar4 = &DAT_006bcf70;
  local_4 = 0xffffffff;
  (**(code **)(*param_1 + 0x4c))(pvVar3,0);
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x6a,0xca,0x1b,0x19,param_1,0xdd02,0x45b,0x45c,puVar2,0,1);
  }
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bce44);
  ExceptionList = puVar4;
  return;
}

