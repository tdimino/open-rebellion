
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall
FUN_005ec310(void *this,uint param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5)

{
  float fVar1;
  
  FUN_005ea030(this,param_1);
  *(undefined ***)this = &PTR_FUN_0066da08;
  *(undefined ***)((int)this + 0xc) = &PTR_LAB_0066d9e8;
  fVar1 = _DAT_0066d838 + _DAT_0066d838;
  *(float *)((int)this + 0xa8) = fVar1;
  *(float *)((int)this + 0xac) = fVar1;
  *(undefined4 *)((int)this + 0xb4) = 0xbf800000;
  *(float *)((int)this + 0xb0) = fVar1;
  *(undefined4 *)((int)this + 0x3c) = param_2;
  *(undefined4 *)((int)this + 0x40) = param_3;
  *(undefined4 *)((int)this + 0x44) = param_4;
  *(undefined4 *)((int)this + 0x48) = param_5;
  return this;
}

