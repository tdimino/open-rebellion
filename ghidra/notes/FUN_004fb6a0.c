
bool __thiscall FUN_004fb6a0(void *this,undefined4 param_1,void *param_2)

{
  int iVar1;
  void *this_00;
  bool bVar2;
  void *pvVar3;
  int iStack_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006400e8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(*(int *)this + 0x84))(param_2);
  bVar2 = iVar1 != 0;
  FUN_004fcd00(&stack0xffffffd0,this,4);
  puStack_8 = (undefined1 *)0x0;
  FUN_00513120((int)&stack0xffffffd0);
  while (iStack_14 != 0) {
    pvVar3 = param_2;
    this_00 = (void *)FUN_0052bed0((int)&stack0xffffffd0);
    iVar1 = FUN_004f8240(this_00,pvVar3);
    if ((iVar1 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_005130d0((int)&stack0xffffffd0);
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_004fcd80((undefined4 *)&stack0xffffffd0);
  ExceptionList = pvStack_10;
  return bVar2;
}

