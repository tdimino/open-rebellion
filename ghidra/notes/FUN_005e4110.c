
void __fastcall FUN_005e4110(int *param_1)

{
  undefined4 uVar1;
  uint *puVar2;
  void *pvVar3;
  undefined *puVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654f72;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(6);
  puVar2 = (uint *)FUN_005ff440((int)param_1);
  pvVar3 = (void *)FUN_00618b70(0xb8);
  local_4 = 0;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_0060e150(pvVar3,0x22,0x51,0x25,10,(int)param_1,0x2ff0000,0x20000ff,0x54000000);
  }
  param_1[0x28] = (int)pvVar3;
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    *(undefined4 *)((int)pvVar3 + 0x24) = 0xdd05;
    FUN_0060e400((void *)param_1[0x28],100,0);
    FUN_005ffce0((void *)param_1[0x28],0);
    FUN_00600a40(param_1,(void *)param_1[0x28],&DAT_006bd5e4);
    (**(code **)(*(int *)param_1[0x28] + 4))(5);
  }
  pvVar3 = (void *)FUN_00618b70(0xb8);
  local_4 = 1;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_0060e150(pvVar3,0x67,0x51,0x25,10,(int)param_1,0x2ff0000,0x20000ff,0x54000000);
  }
  local_4 = 0xffffffff;
  param_1[0x29] = (int)pvVar3;
  if (pvVar3 != (void *)0x0) {
    *(undefined4 *)((int)pvVar3 + 0x24) = 0xdd06;
    FUN_0060e400((void *)param_1[0x29],100,0);
    FUN_005ffce0((void *)param_1[0x29],0);
    FUN_00600a40(param_1,(void *)param_1[0x29],&DAT_006bd4b0);
    (**(code **)(*(int *)param_1[0x29] + 4))(5);
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 2;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,7,10,0xb,0x35,param_1,0xdd03,0x44d,0x44e,puVar2,0,1);
  }
  local_4 = 0xffffffff;
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bd648);
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x84,10,0xb,0x35,param_1,0xdd04,0x44f,0x450,puVar2,0,1);
  }
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bd628);
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x50,0xd2,0x3a,0x16,param_1,0xdd01,0x451,0x452,puVar2,0,1);
  }
  puVar4 = &DAT_006bd658;
  (**(code **)(*param_1 + 0x4c))(pvVar3,0);
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0xc,0xd2,0x3a,0x16,param_1,0xdd02,0x453,0x454,puVar2,0,1);
  }
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bd574);
  FUN_00600b30(param_1);
  FUN_00600b30(param_1);
  FUN_00600b30(param_1);
  FUN_00600b30(param_1);
  FUN_00600b30(param_1);
  ExceptionList = puVar4;
  return;
}

