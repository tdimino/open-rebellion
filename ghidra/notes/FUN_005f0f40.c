
void __fastcall FUN_005f0f40(int *param_1)

{
  undefined4 uVar1;
  uint *puVar2;
  void *pvVar3;
  void *pvVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006554f6;
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
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x42,0xca,0x1b,0x19,param_1,0xdd01,0x459,0x45a,puVar2,0,1);
  }
  pvVar4 = (void *)0x0;
  local_4 = 0xffffffff;
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bdbd8);
  pvVar3 = (void *)FUN_00618b70(0x114);
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x6a,0xca,0x1b,0x19,param_1,0xdd02,0x45b,0x45c,puVar2,0,1);
  }
  (**(code **)(*param_1 + 0x4c))(pvVar3,0,&DAT_006bdad0);
  ExceptionList = pvVar4;
  return;
}

