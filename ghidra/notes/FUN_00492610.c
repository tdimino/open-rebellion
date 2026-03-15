
void __thiscall FUN_00492610(void *this,void *param_1)

{
  int iVar1;
  undefined4 uVar2;
  int local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635e90;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = 0;
  FUN_004c4c50(this,param_1);
  FUN_005f4d90(param_1,(int)this + 100);
  FUN_005f4d90(param_1,(int)this + 0x7c);
  FUN_005f4d90(param_1,(int)this + 0x80);
  FUN_005f4d90(param_1,(int)this + 0x84);
  FUN_004ece90((undefined4 *)((int)this + 0x78),param_1);
  FUN_004ece90((void *)((int)this + 0x74),param_1);
  FUN_005f4d90(param_1,&local_18);
  if (local_18 != 0) {
    iVar1 = FUN_004f5cd0(0x250);
    *(int *)((int)this + 0x90) = iVar1;
    if (iVar1 != 0) {
      *(undefined4 *)(iVar1 + 0x20) = *(undefined4 *)((int)this + 0x20);
      FUN_004f44b0(local_14,(undefined4 *)((int)this + 0x78),0);
      (**(code **)(**(int **)((int)this + 0x90) + 0x24))(local_14);
      uVar2 = FUN_004ece30((undefined4 *)&stack0x00000000);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
      (**(code **)(**(int **)((int)this + 0x90) + 0x2c))(uVar2);
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return;
}

