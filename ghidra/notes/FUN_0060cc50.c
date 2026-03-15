
void * __thiscall
FUN_0060cc50(void *this,undefined4 param_1,int param_2,undefined4 param_3,undefined4 param_4,
            int param_5,int param_6,undefined4 param_7)

{
  FUN_005ff1c0(this,param_1,param_3,param_4,param_5,param_6,0,0,0);
  *(undefined ***)this = &PTR_FUN_0066e1f0;
  *(undefined4 *)((int)this + 0xac) = param_7;
  *(int *)((int)this + 0xa0) = param_2;
  *(undefined4 *)((int)this + 0x9c) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0xa8) = *(undefined4 *)(param_2 + 0xb4);
  *(undefined4 *)((int)this + 0x50) = 0x92000000;
  *(undefined4 *)((int)this + 0x54) = 8;
  return this;
}

