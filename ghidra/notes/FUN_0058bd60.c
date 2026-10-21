
void __thiscall
FUN_0058bd60(void *this,int param_1,undefined4 param_2,undefined4 param_3,uint *param_4)

{
  uint uVar1;
  
  *param_4 = 0;
  if (param_1 != 0) {
    uVar1 = (uint)(*(int *)((int)this + 8) == *(int *)((int)this + 0xc));
    *param_4 = uVar1;
    if (uVar1 != 0) {
      *(int *)((int)this + 0x10) = param_1;
      return;
    }
    *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
  }
  return;
}

