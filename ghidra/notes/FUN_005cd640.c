
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005cd640(void *this,float param_1)

{
  float fVar1;
  
  fVar1 = param_1 * _DAT_0066d030;
  *(float *)((int)this + 0x34) = *(float *)((int)this + 0x70) * fVar1 + *(float *)((int)this + 0x34)
  ;
  *(float *)((int)this + 0x38) = *(float *)((int)this + 0x74) * fVar1 + *(float *)((int)this + 0x38)
  ;
  *(float *)((int)this + 0x30) = *(float *)((int)this + 0x30) + *(float *)((int)this + 0x6c) * fVar1
  ;
  *(undefined4 *)((int)this + 0x3c) = 0xbf800000;
  return;
}

