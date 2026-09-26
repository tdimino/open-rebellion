
uint __thiscall FUN_004f6b90(void *this,uint param_1)

{
  bool bVar1;
  
  if (((int)param_1 < 1) || (4 < (int)param_1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (!bVar1) {
    return param_1;
  }
  switch(param_1) {
  case 1:
    return *(uint *)((int)this + 0x50) & 1;
  case 2:
    return *(uint *)((int)this + 0x50) >> 4 & 1;
  case 3:
    return *(uint *)((int)this + 0x50) >> 6 & 1;
  case 4:
    return 1;
  default:
    return 0;
  }
}

