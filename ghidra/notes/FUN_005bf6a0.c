
void __thiscall FUN_005bf6a0(void *this,void *param_1)

{
  undefined4 local_24 [2];
  uint local_1c;
  undefined4 uStack_18;
  undefined4 local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653800;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005acf80(this,param_1);
  FUN_005f5fa0(local_24);
  local_4 = 0;
  FUN_005ce6c0((void *)((int)this + 0x908),param_1);
  FUN_005ce7b0((void *)((int)this + 0x908),local_24);
  FUN_005ce6c0((void *)((int)this + 0x914),param_1);
  FUN_005ce7b0((void *)((int)this + 0x914),local_24);
  FUN_005ce6c0((void *)((int)this + 0x920),param_1);
  FUN_005ce7b0((void *)((int)this + 0x920),local_24);
  FUN_005ce6c0((void *)((int)this + 0x92c),param_1);
  FUN_005ce7b0((void *)((int)this + 0x92c),local_24);
  FUN_005ce6c0((void *)((int)this + *(int *)((int)this + 0x9ec) * 0xc + 0x938),param_1);
  FUN_005ce7b0((void *)((int)this + *(int *)((int)this + 0x9ec) * 0xc + 0x938),local_24);
  FUN_005f5fa0(&local_14);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005f6150(&local_14,param_1);
  uStack_18 = 0;
  local_1c = *(int *)((int)this + *(int *)((int)this + 0x9ec) * 4 + 0x868) + 1;
  FUN_005ce7e0((void *)((int)this + *(int *)((int)this + 0x9ec) * 0xc + 0x908),(float)local_1c);
  (**(code **)(*(int *)this + 0x7c))(10000);
  puStack_8 = (undefined1 *)((uint)puStack_8 & 0xffffff00);
  FUN_005f5fe0(&uStack_18);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_005f5fe0((undefined4 *)&stack0xffffffd8);
  ExceptionList = pvStack_10;
  return;
}

