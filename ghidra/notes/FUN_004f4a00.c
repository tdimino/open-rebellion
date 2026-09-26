
bool __thiscall FUN_004f4a00(void *this,void *param_1)

{
  void *this_00;
  void *pvVar1;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 auStack_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063f7f0;
  pvStack_c = ExceptionList;
  local_24 = 1;
  local_20 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_24);
  local_4 = 0xffffffff;
  FUN_00619730();
  this_00 = (void *)(**(code **)(*(int *)this + 0x48))();
  if (this_00 != (void *)0x0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      (**(code **)(**(int **)((int)this + 0x40) + 4))();
      FUN_00541e70(auStack_1c);
      local_4 = 1;
      FUN_0051fcb0(this,auStack_1c);
      FUN_0054f3e0(this_00,auStack_1c);
      *(undefined4 *)((int)this_00 + 0x50) = *(undefined4 *)((int)this + 0x44);
      *(undefined4 *)((int)this_00 + 0x54) = *(undefined4 *)((int)this + 0x48);
      pvVar1 = (void *)(**(code **)(*(int *)this + 0x28))();
      FUN_004f43b0((void *)((int)this_00 + 100),pvVar1);
      FUN_004f43b0((void *)((int)this_00 + 0x6c),(void *)((int)this + 0x58));
      local_24 = 0xf8000006;
      local_4._0_1_ = 2;
      FUN_004f26d0((void *)((int)this_00 + 0x3c),&local_24);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_0054ee30(*(void **)((int)this + 0x40),(int)this_00);
      local_4 = 0xffffffff;
      FUN_00541ea0();
    }
  }
  else {
    local_24 = 1;
    local_20 = 1;
    local_4 = 3;
    FUN_00520580(param_1,&local_24);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return this_00 != (void *)0x0;
}

