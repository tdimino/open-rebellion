
int FUN_00624820(void)

{
  DWORD DVar1;
  DWORD DVar2;
  uint uVar3;
  int iVar4;
  DWORD *pDVar5;
  HANDLE hFile;
  BOOL BVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  undefined4 *puVar10;
  uint in_stack_00001008;
  int in_stack_0000100c;
  
  FUN_00627300();
  iVar8 = 0;
  DVar1 = FUN_0061bc00(in_stack_00001008,0,1);
  if ((DVar1 == 0xffffffff) || (DVar2 = FUN_0061bc00(in_stack_00001008,0,2), DVar2 == 0xffffffff)) {
    return -1;
  }
  uVar9 = in_stack_0000100c - DVar2;
  if ((int)uVar9 < 1) {
    if ((int)uVar9 < 0) {
      FUN_0061bc00(in_stack_00001008,in_stack_0000100c,0);
      hFile = (HANDLE)FUN_00624730(in_stack_00001008);
      BVar6 = SetEndOfFile(hFile);
      iVar8 = (BVar6 != 0) - 1;
      if (iVar8 == -1) {
        pDVar5 = FUN_0061c250();
        *pDVar5 = 0xd;
        DVar2 = GetLastError();
        pDVar5 = FUN_0061c260();
        *pDVar5 = DVar2;
      }
    }
    FUN_0061bc00(in_stack_00001008,DVar1,0);
    return iVar8;
  }
  puVar10 = (undefined4 *)&stack0x00000004;
  for (iVar7 = 0x400; iVar7 != 0; iVar7 = iVar7 + -1) {
    *puVar10 = 0;
    puVar10 = puVar10 + 1;
  }
  iVar7 = FUN_0061c8e0(in_stack_00001008,0x8000);
  while( true ) {
    uVar3 = 0x1000;
    if ((int)uVar9 < 0x1000) {
      uVar3 = uVar9;
    }
    iVar4 = FUN_0061b6c0(in_stack_00001008,&stack0x00000004,uVar3);
    if (iVar4 == -1) break;
    uVar9 = uVar9 - iVar4;
    if ((int)uVar9 < 1) {
LAB_006248da:
      FUN_0061c8e0(in_stack_00001008,iVar7);
      FUN_0061bc00(in_stack_00001008,DVar1,0);
      return iVar8;
    }
  }
  pDVar5 = FUN_0061c260();
  if (*pDVar5 == 5) {
    pDVar5 = FUN_0061c250();
    *pDVar5 = 0xd;
  }
  iVar8 = -1;
  goto LAB_006248da;
}

