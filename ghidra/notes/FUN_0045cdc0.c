
void __thiscall FUN_0045cdc0(void *this,void *param_1)

{
  uint *puVar1;
  void *pvVar2;
  void *pvVar3;
  int iVar4;
  uint uVar5;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar3 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630bb0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  puVar1 = FUN_004025b0(param_1,(uint *)&param_1);
  local_4 = 0;
  pvVar2 = (void *)FUN_0060a860((void *)((int)this + 0x174),(ushort)*puVar1 | 0x40000);
  local_4 = 0xffffffff;
  FUN_00619730();
  uVar5 = *(uint *)((int)pvVar3 + 0x24) >> 6 & 3;
  pvVar3 = FUN_0053b6e0(local_2c,pvVar3,3);
  local_4 = 1;
  iVar4 = FUN_00513180((int)pvVar3);
  local_4 = 0xffffffff;
  FUN_0053b7e0(local_2c);
  if (iVar4 == 0) {
    uVar5 = 0;
  }
  FUN_0045d140(this,pvVar2,uVar5,iVar4);
  ExceptionList = pvStack_c;
  return;
}

