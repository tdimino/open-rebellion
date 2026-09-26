
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

byte __cdecl FUN_00611330(uint param_1)

{
  int iVar1;
  ushort uVar2;
  undefined4 local_14;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  local_c = 0;
  local_8 = 0;
  local_4 = 0;
  _DAT_006be5f0 = 0;
  uVar2 = (ushort)(param_1 >> 8) & 0xff;
  _DAT_006be5e0 = CONCAT22(uVar2,1);
  local_14 = 0x14;
  local_10 = 0x81;
  _DAT_006be5e8 = (int)((uint)uVar2 * (uint)((ushort)param_1 & 0xff)) >> 3;
  DAT_006be5ec = CONCAT22((ushort)param_1,(short)_DAT_006be5e8) & 0xffffff;
  _DAT_006be5e4 = param_1 >> 0x10;
  _DAT_006be5e8 = _DAT_006be5e4 * _DAT_006be5e8;
  if (DAT_006be66c != (int *)0x0) {
    iVar1 = (**(code **)(*DAT_006be66c + 0x38))(DAT_006be66c,&DAT_006be5e0);
    return -(iVar1 != 0) & 10;
  }
  iVar1 = (**(code **)(*DAT_006be668 + 0xc))(DAT_006be668,&local_14,&DAT_006be66c,0);
  if (iVar1 == 0) {
    iVar1 = (**(code **)(*DAT_006be66c + 0x38))(DAT_006be66c,&DAT_006be5e0);
    return -(iVar1 != 0) & 10;
  }
  return 8;
}

