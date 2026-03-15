
void * __thiscall
FUN_005eb860(void *this,uint param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  FUN_005ea030(this,param_1);
  *(undefined ***)this = &PTR_FUN_0066d9a0;
  *(undefined ***)((int)this + 0xc) = &PTR_LAB_0066d980;
  *(undefined4 *)((int)this + 0x14) = DAT_006bd88c;
  *(undefined4 *)((int)this + 0x40) = param_3;
  *(undefined4 *)((int)this + 0x3c) = param_2;
  *(undefined4 *)((int)this + 0x44) = param_4;
  *(undefined4 *)((int)this + 0x48) = 0xbf800000;
  return this;
}

