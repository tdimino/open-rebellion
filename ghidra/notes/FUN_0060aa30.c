
bool FUN_0060aa30(ushort *param_1,int param_2,LPSTR param_3)

{
  HINSTANCE hInstance;
  int iVar1;
  bool bVar2;
  
  hInstance = (HINSTANCE)FUN_005fefd0(param_1[1]);
  bVar2 = false;
  if (hInstance != (HINSTANCE)0x0) {
    iVar1 = LoadStringA(hInstance,(uint)*param_1,param_3,param_2);
    bVar2 = iVar1 != 0;
  }
  return bVar2;
}

