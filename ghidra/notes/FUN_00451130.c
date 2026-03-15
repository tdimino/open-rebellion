
void * __thiscall
FUN_00451130(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,int param_7,undefined4 param_8,undefined4 param_9)

{
  FUN_004aa960(this,param_1,param_2,param_3,param_4,param_5,param_7,param_8,param_9);
  *(undefined4 *)((int)this + 0x14c) = param_9;
  *(undefined4 *)((int)this + 0x154) = param_6;
  *(undefined4 *)((int)this + 0x160) = 0;
  *(undefined4 *)((int)this + 0x164) = 0;
  *(undefined4 *)((int)this + 0x168) = 0;
  *(undefined4 *)((int)this + 0x16c) = 0;
  *(undefined ***)this = &PTR_FUN_00659d80;
  *(int *)((int)this + 0x150) = param_7;
  return this;
}

