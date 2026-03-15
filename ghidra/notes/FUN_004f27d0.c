
longlong __thiscall FUN_004f27d0(void *this,int param_1)

{
  bool bVar1;
  undefined4 uVar2;
  uint uVar3;
  
  uVar2 = 0;
  if ((param_1 < 1) || (3 < param_1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  uVar3 = 0;
  if ((bVar1) && (uVar3 = *(uint *)((int)this + 0x2c), uVar3 != 0)) {
    if ((*(byte *)((int)this + 0x50) & 0x40) == 0) {
      return CONCAT44(uVar3,1);
    }
    if (param_1 == 1) {
      uVar2 = *(undefined4 *)(uVar3 + 0x40);
    }
    else {
      if (param_1 == 2) {
        return CONCAT44(uVar3,*(undefined4 *)(uVar3 + 0x44));
      }
      if (param_1 == 3) {
        if ((*(int *)(uVar3 + 0x40) != 0) && (*(int *)(uVar3 + 0x44) != 0)) {
          return CONCAT44(uVar3,1);
        }
        return (ulonglong)uVar3 << 0x20;
      }
    }
  }
  return CONCAT44(uVar3,uVar2);
}

