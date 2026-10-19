
uint __fastcall FUN_00534fa0(int *param_1)

{
  bool bVar1;
  uint uVar2;
  uint uVar3;
  undefined3 extraout_var;
  int *local_4;
  
  local_4 = param_1;
  uVar2 = thunk_FUN_004f9700(param_1);
  uVar3 = 0;
  if (uVar2 != 0) {
    uVar3 = (uint)~param_1[0x1e] >> 6 & 1;
  }
  local_4 = (int *)0x0;
  uVar2 = 0;
  if (uVar3 != 0) {
    bVar1 = FUN_004f6b50(param_1,(int *)&local_4);
    uVar2 = CONCAT31(extraout_var,bVar1);
    if (((uVar2 != 0) && ((*(byte *)(param_1 + 0x1e) & 0x80) == 0)) &&
       (((param_1[0x14] & 8U) == 0 || ((param_1[0x14] & 0x4000U) == 0)))) {
      uVar2 = (uint)~local_4[0x14] >> 3 & 1;
    }
  }
  return uVar2;
}

