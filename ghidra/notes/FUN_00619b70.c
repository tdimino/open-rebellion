
undefined4 __cdecl FUN_00619b70(int param_1)

{
  int iVar1;
  int iVar2;
  BOOL BVar3;
  undefined *puVar4;
  DWORD *pDVar5;
  
  iVar1 = param_1;
  if ((0 < param_1) && (param_1 < 0x20)) {
    FUN_0061d9d0(0xc);
    iVar2 = param_1;
    param_1 = CONCAT31(0x3a,(char)iVar1 + '@');
    iVar1 = param_1;
    param_1._3_1_ = SUB41(iVar2,3);
    param_1._0_3_ = (uint3)(ushort)iVar1;
    BVar3 = SetCurrentDirectoryA((LPCSTR)&param_1);
    if (BVar3 != 0) {
      FUN_0061da50(0xc);
      return 0;
    }
    puVar4 = (undefined *)GetLastError();
    FUN_0061c1d0(puVar4);
    FUN_0061da50(0xc);
    return 0xffffffff;
  }
  pDVar5 = FUN_0061c250();
  *pDVar5 = 0xd;
  pDVar5 = FUN_0061c260();
  *pDVar5 = 0xf;
  return 0xffffffff;
}

