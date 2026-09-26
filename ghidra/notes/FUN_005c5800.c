
void __fastcall FUN_005c5800(void *param_1)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  void *pvVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653c3d;
  local_c = ExceptionList;
  uVar1 = *(undefined4 *)((int)param_1 + 0x1c);
  iVar2 = *(int *)((int)param_1 + 0x30);
  iVar3 = *(int *)((int)param_1 + 0x34);
  ExceptionList = &local_c;
  pvVar4 = (void *)FUN_00618b70(0x9c);
  local_4 = 0;
  if (pvVar4 == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005e9b60(pvVar4,uVar1,0,0,iVar2,iVar3,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005c5a20(param_1,pvVar4);
  pvVar4 = (void *)FUN_00618b70(0xc4);
  local_4 = 1;
  if (pvVar4 == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005e6b90(pvVar4,uVar1,600,600,iVar2,iVar3,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005c5a20(param_1,pvVar4);
  pvVar4 = (void *)FUN_00618b70(0xc0);
  local_4 = 2;
  if (pvVar4 == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005e3c80(pvVar4,uVar1,600,600,iVar2,iVar3,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005c5a20(param_1,pvVar4);
  pvVar4 = (void *)FUN_00618b70(0xc0);
  local_4 = 3;
  if (pvVar4 == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005e1170(pvVar4,uVar1,600,600,iVar2,iVar3,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005c5a20(param_1,pvVar4);
  pvVar4 = (void *)FUN_00618b70(200);
  local_4 = 4;
  if (pvVar4 == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005dee00(pvVar4,uVar1,600,600,iVar2,iVar3,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005c5a20(param_1,pvVar4);
  pvVar4 = (void *)FUN_00618b70(0xcc);
  local_4 = 5;
  if (pvVar4 == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005dc960(pvVar4,uVar1,600,600,iVar2,iVar3,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005c5a20(param_1,pvVar4);
  pvVar4 = (void *)FUN_00618b70(0xa0);
  local_4 = 6;
  if (pvVar4 == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005da950(pvVar4,uVar1,600,600,iVar2,iVar3,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005c5a20(param_1,pvVar4);
  ExceptionList = local_c;
  return;
}

