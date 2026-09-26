
void __thiscall FUN_0042b1d0(void *this,uint param_1,uint param_2,uint param_3,int param_4)

{
  HMODULE hModule;
  HRSRC hResInfo;
  HGLOBAL hResData;
  int iVar1;
  undefined2 *puVar2;
  
  puVar2 = (undefined2 *)0x0;
  hResData = (HGLOBAL)0x0;
  iVar1 = (-(uint)(param_4 != 0) & 4) + 9;
  hModule = (HMODULE)FUN_005fefd0(iVar1);
  hResInfo = FindResourceA(hModule,(LPCSTR)(param_1 & 0xffff),(LPCSTR)0xa);
  if (hResInfo != (HRSRC)0x0) {
    hResData = LoadResource(hModule,hResInfo);
    if (hResData != (HGLOBAL)0x0) {
      puVar2 = LockResource(hResData);
    }
  }
  if ((puVar2 != (undefined2 *)0x0) && (*(int **)((int)this + 0x128) != (int *)0x0)) {
    if (param_2 == 0) {
      param_2 = (uint)(ushort)puVar2[2];
    }
    if (param_3 == 0) {
      param_3 = (uint)(ushort)puVar2[3];
    }
    param_4 = CONCAT22((short)iVar1,puVar2[1]);
    (**(code **)(**(int **)((int)this + 0x128) + 0x38))(0xdead,&param_4,*puVar2,param_2,param_3);
    FreeResource(hResData);
  }
  return;
}

