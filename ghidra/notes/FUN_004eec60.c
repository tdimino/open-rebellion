
void __thiscall FUN_004eec60(void *this,void *param_1)

{
  uint uVar1;
  
  uVar1 = FUN_004edc80((int)this);
  if ((*(byte *)((int)this + 0xac) & 1) != 0) {
    if (uVar1 != 3) {
      FUN_004f6f40(this,2 - (uVar1 != 1),param_1);
      return;
    }
    uVar1 = 3;
  }
  FUN_004f6f40(this,uVar1,param_1);
  return;
}

