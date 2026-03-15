
bool FUN_00561910(int param_1,int param_2,void *param_3)

{
  void *this;
  uint uVar1;
  bool bVar2;
  
  if ((param_2 < 1) || (3 < param_2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if (((bVar2) && (bVar2 = param_2 != 3, bVar2)) && (param_1 != 0)) {
    for (this = (void *)FUN_00505120(); this != (void *)0x0; this = (void *)FUN_00562a50(this)) {
      uVar1 = FUN_004ef1e0(this,param_1,param_2,param_3);
      if ((uVar1 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
  }
  return bVar2;
}

