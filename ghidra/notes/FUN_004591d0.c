
void * __thiscall
FUN_004591d0(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            int param_6,uint *param_7,undefined4 param_8,undefined4 param_9)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006306b4;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ac120(this,param_1,param_2,param_3,param_4,param_5,param_6,param_8,param_7);
  local_4 = 0;
  FUN_0060a790((void *)((int)this + 0x164),1);
  local_4._0_1_ = 1;
  FUN_0060a790((void *)((int)this + 0x174),1);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_004f4340((undefined4 *)((int)this + 0x184));
  *(undefined4 *)((int)this + 0x18c) = 0;
  *(undefined4 *)((int)this + 400) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x144) = param_9;
  *(undefined4 *)((int)this + 0x198) = 0;
  *(undefined4 *)((int)this + 0x19c) = 0;
  *(undefined4 *)((int)this + 0x14c) = 0xfffffc18;
  *(undefined4 *)((int)this + 0x150) = 0xfffffc18;
  *(undefined4 *)((int)this + 0x1a0) = 0;
  *(undefined4 *)((int)this + 0x160) = 0;
  *(undefined ***)this = &PTR_FUN_00659f18;
  *(int *)((int)this + 0x194) = param_6;
  ExceptionList = local_c;
  return this;
}

