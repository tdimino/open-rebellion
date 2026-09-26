
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_006214a0(void)

{
  undefined4 *puVar1;
  LPSTR *ppCVar2;
  int iStack_8;
  int iStack_4;
  
  GetModuleFileNameA((HMODULE)0x0,(LPSTR)&lpFilename_006be860,0x104);
  _DAT_006be7b4 = &lpFilename_006be860;
  ppCVar2 = DAT_006bfe4c;
  if (*(char *)DAT_006bfe4c == '\0') {
    ppCVar2 = &lpFilename_006be860;
  }
  FUN_00621540((byte *)ppCVar2,(undefined4 *)0x0,(byte *)0x0,&iStack_8,&iStack_4);
  puVar1 = (undefined4 *)FUN_00619170(iStack_4 + iStack_8 * 4);
  if (puVar1 == (undefined4 *)0x0) {
    __amsg_exit(8);
  }
  FUN_00621540((byte *)ppCVar2,puVar1,(byte *)(puVar1 + iStack_8),&iStack_8,&iStack_4);
  _DAT_006be79c = puVar1;
  _DAT_006be798 = iStack_8 + -1;
  return;
}

