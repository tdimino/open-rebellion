
void * __thiscall
FUN_005faba0(void *this,undefined4 *param_1,uint param_2,byte param_3,undefined1 param_4,
            undefined1 param_5,int param_6)

{
  void *this_00;
  void *this_01;
  undefined4 uVar1;
  uint uVar2;
  undefined4 *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655d43;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(this);
  *(undefined ***)this = &PTR_FUN_0066de34;
  local_4 = 0;
  this_00 = (void *)FUN_005f7cc0();
  *(undefined4 *)((int)this + 0x1c) = 0;
  puVar3 = (undefined4 *)((int)this + 0x29);
  for (uVar2 = (param_2 & 0xffff) >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar3 = *param_1;
    param_1 = param_1 + 1;
    puVar3 = puVar3 + 1;
  }
  for (uVar2 = param_2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *(undefined1 *)puVar3 = *(undefined1 *)param_1;
    param_1 = (undefined4 *)((int)param_1 + 1);
    puVar3 = (undefined4 *)((int)puVar3 + 1);
  }
  *(undefined1 *)((int)this + 0x28) = param_5;
  *(undefined1 *)((int)this + 0x27) = param_4;
  *(short *)((int)this + 0x24) = (short)param_2;
  *(char *)((int)this + 0x26) = (char)param_6;
  DAT_006be39c = DAT_006be39c + 1;
  uVar2 = DAT_006be39c & 0xf0ffffff | (param_3 & 0xf) << 0x18;
  this_01 = (void *)FUN_00618b70(0x88);
  local_4._0_1_ = 1;
  if (this_01 == (void *)0x0) {
    uVar1 = 0;
  }
  else {
    uVar1 = FUN_005fb2d0(this_01,uVar2);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  *(undefined4 *)((int)this + 0x1c) = uVar1;
  FUN_005f5600(this,uVar2);
  if (param_6 != 0) {
    FUN_005f5440(this_00,this);
  }
  *(uint *)((int)this + 0x20) = uVar2;
  ExceptionList = local_c;
  return this;
}

