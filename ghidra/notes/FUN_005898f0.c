
bool __thiscall FUN_005898f0(void *this,uint param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  bool bVar2;
  
  if ((*(int *)((int)this + 0x14) == 0) || (*(int *)((int)this + 0x10) == 0)) {
    bVar2 = false;
  }
  else {
    bVar2 = ((byte)*(undefined4 *)(*(int *)((int)this + 0x14) + 0x24) & 0xc0) != 0xc0;
    if (bVar2) {
      *(undefined4 *)((int)this + 0x44) = 1;
      bVar2 = FUN_00589970(this,param_1);
      *(undefined4 *)((int)this + 0x44) = 0;
      bVar1 = FUN_00589970(this,param_1);
      if ((CONCAT31(extraout_var_00,bVar1) != 0) && (CONCAT31(extraout_var,bVar2) != 0)) {
        return true;
      }
      bVar2 = false;
    }
  }
  return bVar2;
}

