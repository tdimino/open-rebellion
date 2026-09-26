
void * __thiscall FUN_005faed0(void *this,undefined4 *param_1,uint param_2,byte param_3,int param_4)

{
  uint uVar1;
  void *this_00;
  void *pvVar2;
  undefined4 uVar3;
  uint uVar4;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  uVar4 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655dae;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(this);
  *(undefined ***)this = &PTR_FUN_0066de38;
  local_4 = 0;
  this_00 = (void *)FUN_005f7cc0();
  *(undefined4 *)((int)this + 0x20) = 0;
  uVar1 = param_2 + 0x1f5;
  *(undefined1 *)((int)this + 0x1d) = 0;
  param_2 = 1;
  *(char *)((int)this + 0x1c) = (char)((ulonglong)uVar1 * 0x828cbfbf >> 0x28);
  uVar1 = param_2;
  while (uVar4 != 0) {
    if (uVar4 < 0x1f7) {
      pvVar2 = (void *)FUN_00618b70(0x220);
      local_4._0_1_ = 1;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005faba0(pvVar2,param_1,uVar4,param_3,(char)param_2,
                              *(undefined1 *)((int)this + 0x1c),param_4);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (*(int *)((int)this + 0x20) == 0) {
        if (*(int *)((int)pvVar2 + 0x1c) == 0) {
          uVar3 = 0;
        }
        else {
          uVar3 = *(undefined4 *)(*(int *)((int)pvVar2 + 0x1c) + 4);
        }
        *(undefined4 *)((int)this + 0x20) = uVar3;
      }
      uVar4 = 0;
      if (*(char *)((int)pvVar2 + 0x26) == '\0') {
        FUN_005f5440(this_00,pvVar2);
      }
    }
    else {
      pvVar2 = (void *)FUN_00618b70(0x220);
      local_4._0_1_ = 2;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005faba0(pvVar2,param_1,0x1f6,param_3,(char)param_2,
                              *(undefined1 *)((int)this + 0x1c),param_4);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (*(int *)((int)this + 0x20) == 0) {
        if (*(int *)((int)pvVar2 + 0x1c) == 0) {
          uVar3 = 0;
        }
        else {
          uVar3 = *(undefined4 *)(*(int *)((int)pvVar2 + 0x1c) + 4);
        }
        *(undefined4 *)((int)this + 0x20) = uVar3;
      }
      param_1 = (undefined4 *)((int)param_1 + 0x1f6);
      uVar4 = uVar4 - 0x1f6;
      if (*(char *)((int)pvVar2 + 0x26) == '\0') {
        FUN_005f5440(this_00,pvVar2);
      }
      param_2._0_1_ = (char)uVar1;
      param_2 = (uint)(byte)((char)param_2 + 1);
      uVar1 = param_2;
    }
  }
  FUN_005f5600(this,*(undefined4 *)((int)this + 0x20));
  ExceptionList = local_c;
  return this;
}

