
undefined4 __cdecl FUN_00611890(int param_1,HMODULE param_2,undefined4 *param_3)

{
  HRSRC hResInfo;
  HGLOBAL hResData;
  LPVOID pvVar1;
  void *pvVar2;
  undefined4 uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656e2b;
  local_c = ExceptionList;
  uVar3 = 1;
  if ((*(int *)(param_1 + 4) != 0) && (param_2 != (HMODULE)0x0)) {
    ExceptionList = &local_c;
    hResInfo = FindResourceA(param_2,(LPCSTR)(uint)*(ushort *)(param_1 + 4),(LPCSTR)&lpType_006ac954
                            );
    if (hResInfo != (HRSRC)0x0) {
      hResData = LoadResource(param_2,hResInfo);
      if (hResData != (HGLOBAL)0x0) {
        pvVar1 = LockResource(hResData);
        if (pvVar1 != (LPVOID)0x0) {
          if ((*(uint *)(param_1 + 0x10) & 7) == 1) {
            pvVar2 = (void *)FUN_00618b70(0x58);
            uVar3 = 0;
            uStack_4 = 0;
            if (pvVar2 == (void *)0x0) {
              pvVar2 = (void *)0x0;
            }
            else {
              pvVar2 = FUN_00612200(pvVar2,(uint)pvVar1,2,param_1);
            }
            uStack_4 = 0xffffffff;
            *param_3 = pvVar2;
          }
          else {
            uVar3 = 3;
          }
          FreeResource(hResData);
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

