
void __fastcall FUN_005e1550(int *param_1)

{
  undefined4 uVar1;
  uint *puVar2;
  void *pvVar3;
  void *pvVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654d86;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(6);
  puVar2 = (uint *)FUN_005ff440((int)param_1);
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 0;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0xc,0xd2,0x3a,0x16,param_1,0xdd02,0x453,0x454,puVar2,0,1);
  }
  pvVar4 = (void *)0x0;
  local_4 = 0xffffffff;
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bd364);
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x50,0xd2,0x3a,0x16,param_1,0xdd01,0x451,0x452,puVar2,0,1);
  }
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bd448);
  ExceptionList = pvVar4;
  return;
}

