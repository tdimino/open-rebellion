
bool __cdecl FUN_00515c00(uint *param_1,uint *param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  bool bVar2;
  void *this;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined4 local_c;
  
  bVar1 = true;
  local_c = (void *)0x0;
  if ((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    local_c = (void *)FUN_00505f60(param_1);
  }
  if (local_c != (void *)0x0) {
    this = (void *)thunk_FUN_00506e80();
    if (this != (void *)0x0) {
      bVar1 = FUN_005613f0(this,param_1,param_2,param_3,param_4);
      bVar1 = CONCAT31(extraout_var,bVar1) != 0;
    }
    bVar2 = FUN_00517aa0(local_c,param_2,param_3,param_4);
    if ((CONCAT31(extraout_var_00,bVar2) != 0) && (bVar1)) {
      return true;
    }
    bVar1 = false;
  }
  return bVar1;
}

