
void * __thiscall
FUN_00452ab0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,uint param_8,uint param_9,
            uint *param_10,undefined4 param_11,undefined4 param_12,undefined4 param_13,
            undefined4 param_14,undefined4 param_15)

{
  FUN_00602150(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_10
               ,0,0);
  *(undefined4 *)((int)this + 0x114) = param_15;
  *(uint **)((int)this + 0x144) = param_10;
  *(undefined4 *)((int)this + 0x11c) = param_11;
  *(undefined ***)this = &PTR_FUN_00659e38;
  *(undefined4 *)((int)this + 0x50) = 0x56000000;
  *(undefined4 *)((int)this + 0x120) = param_12;
  *(undefined4 *)((int)this + 0x118) = 0;
  *(undefined4 *)((int)this + 0x124) = param_13;
  *(undefined4 *)((int)this + 0x128) = param_14;
  return this;
}

