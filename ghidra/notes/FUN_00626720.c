
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int __cdecl FUN_00626720(int param_1,uint param_2)

{
  int iVar1;
  
  if ((param_2._2_2_ & 0x7ff0) != 0x7ff0) {
    if (((param_2 & 0x7ff00000) == 0) && (((param_2 & 0xfffff) != 0 || (param_1 != 0)))) {
      return (-(uint)((param_2 & 0x80000000) != 0) & 0xffffff90) + 0x80;
    }
    if ((double)CONCAT26(param_2._2_2_,CONCAT24((undefined2)param_2,param_1)) == _DAT_0066f2a8) {
      return (-(uint)((param_2 & 0x80000000) != 0) & 0xffffffe0) + 0x40;
    }
    return (-(uint)((param_2 & 0x80000000) != 0) & 0xffffff08) + 0x100;
  }
  iVar1 = FUN_0061fd40(param_1,param_2);
  if (iVar1 == 1) {
    return 0x200;
  }
  if (iVar1 != 2) {
    if (iVar1 != 3) {
      return 1;
    }
    return 2;
  }
  return 4;
}

