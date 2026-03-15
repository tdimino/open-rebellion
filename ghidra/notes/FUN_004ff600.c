
bool __thiscall FUN_004ff600(void *this,int param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  
  iVar1 = FUN_004fb520(this,param_1,param_2);
  bVar2 = iVar1 != 0;
  if (param_1 != 0) {
    iVar1 = FUN_004fe380(this,0,param_2);
    if ((iVar1 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar1 = FUN_004fe4d0(this,0,param_2);
    if ((iVar1 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar1 = FUN_004fe460(this,0,param_2);
    if ((iVar1 != 0) && (bVar2)) {
      return true;
    }
    bVar2 = false;
  }
  return bVar2;
}

